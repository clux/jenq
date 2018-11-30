use chrono::{Utc, TimeZone};
use jenkins_api::{
    JenkinsBuilder,
    Jenkins,
    job::CommonJob,
    build::{Build, CommonBuild},
    action::{ParametersAction, parameters::StringParameterValue},
};

use std::env;
use std::collections::BTreeMap;
pub use failure::{Error, Fail, Context, Backtrace, ResultExt};
/// Error handling convenience type
pub type Result<T> = std::result::Result<T, Error>;

// helpers

fn env_user() -> Result<String> {
    Ok(env::var("JENKINS_API_USER")
        .context(format_err!("JENKINS_API_USER not specified"))?
    )
}
fn env_pass() -> Option<String> {
    env::var("JENKINS_API_TOKEN").ok()
}

fn env_url() -> Result<String> {
    Ok(env::var("JENKINS_URL")
        .context(format_err!("JENKINS_URL not specified"))?
    )
}

fn get_client() -> Result<Jenkins> {
    Ok(JenkinsBuilder::new(&env_url()?)
        .with_user(&env_user()?, env_pass().as_ref().map(String::as_str))
        .build().map_err(|e| {
            format_err!("Failed to create jenkins client {}", e)
        })?
    )
}

fn get_job(client: &Jenkins, job: &str) -> Result<CommonJob> {
    Ok(client.get_job(job)
        .context(format_err!("Failed to get jenkins job {}", job))
    ?)
}

/// Jenkins StringParameters used for job selection/filtering
///
/// Type is used for both result from querying jenkins for what parameters exist,
/// and from arg parser for what parameters we want at what value.
pub type JobParams = BTreeMap<String, String>;

fn get_string_params(b: &CommonBuild) -> JobParams {
    let mut res = BTreeMap::new();
    for a in &b.actions {
        if let Ok(params) = a.as_variant::<ParametersAction>() {
            trace!("got pars {:?}", params);
            for p in params.parameters {
                if let Ok(spar) = p.as_variant::<StringParameterValue>() {
                    res.insert(spar.name.clone(), spar.value.clone());
                }
            }
        }
    }
    res
}


// verifies all requested parameters must exist and match requested values
fn build_satisfies_params(b: &CommonBuild, px: &JobParams) -> bool {
    let params = get_string_params(&b);
    for (par, value) in px {
        if let Some(a) = params.get(par) {
            debug!("Got param: {} = {} (wanted={})", par, a, value);
            if a != value {
                return false
            }
        } else {
            warn!("Parameter {} not found in job", par);
            return false
        }
    }
    true
}


fn find_build_by_parameter(client: &Jenkins, job: &str, px: &JobParams) -> Result<Option<CommonBuild>> {
    let job = get_job(&client, job)?;
    let len = job.builds.len();
    for sbuild in job.builds {
        // ignore errors to fetch full builds here (builds scheduling can fail)
        if let Ok(build) = sbuild.get_full_build(&client) {
            debug!("scanning build :{:?}", build);
            if build_satisfies_params(&build, px) {
                return Ok(Some(build))
            }
        }
    }
    warn!("No completed deploy jobs found for {:?} in the last {} builds", px, len);
    Ok(None)
}

fn find_builds_by_parameter(client: &Jenkins, job: &str, px: &JobParams) -> Result<Vec<CommonBuild>> {
    let job = get_job(&client, job)?;
    let mut builds = vec![];
    let len = job.builds.len();
    for sbuild in job.builds {
        // ignore errors to fetch full builds here (builds scheduling can fail)
        if let Ok(build) = sbuild.get_full_build(&client) {
            debug!("scanning build :{:?}", build);
            if build_satisfies_params(&build, px) {
                builds.push(build);
            }
        }
    }
    if builds.is_empty() {
        warn!("No completed jobs found for {:?} in the last {} builds", px, len);
    }
    Ok(builds)
}

fn find_build_by_nr(client: &Jenkins, job: &str, nr: u32, px: &JobParams) -> Result<Option<CommonBuild>> {
    let job = get_job(&client, job)?;
    let len = job.builds.len();
    for sbuild in job.builds {
        if sbuild.number == nr {
            // handle Err here if we failed because we asked for this number
            let build = sbuild.get_full_build(&client)?;
            if build_satisfies_params(&build, px) {
                return Ok(Some(build))
            }
            else {
                warn!("Build {} found, but it's not for {:?}", nr, px);
                return Ok(None)
            }
        }
    }
    warn!("Build number {} not found for {:?} in last {} builds", nr, px, len);
    Ok(None)
}


/// Print the latest job status
pub fn latest_build(jobname: &str, params: &JobParams) -> Result<()> {
    let client = get_client()?;
    if let Some(build) = find_build_by_parameter(&client, &jobname, params)? {
        let ts = Utc.timestamp((build.timestamp/1000) as i64, 0);
        println!("{}#{} ({}) at {} on {}",
            jobname, build.number, build.queue_id, ts, build.url
        );
    }
    Ok(())
}

/// Print a history for the last jobs matching a set of params
pub fn history(jobname: &str, params: &JobParams) -> Result<()> {
    let client = get_client()?;
    let builds = find_builds_by_parameter(&client, &jobname, params)?;

    if builds.is_empty() {
        return Ok(())
    }
    println!("{0:<6} {1:<20} {2:<9}", "BUILD", "UPDATED", "RESULT");
    for b in builds {
        let ts = Utc.timestamp((b.timestamp/1000) as i64, 0);
        let stamp = ts.format("%Y-%m-%d %H:%M:%S").to_string();
        let link = format!("\x1B]8;;{}\x07{}\x1B]8;;\x07", b.url, b.number);
        // not aligning the build because it's full of escape codes for the link
        println!("{0}   {1:<20} {2:<9?}", link, stamp, b.result);
        // TODO: maybe add parameter values to table?
    }
    Ok(())
}

/// Print the consoleText from the latest job matching a set of params
pub fn latest_console(jobname: &str, params: &JobParams) -> Result<()> {
    let client = get_client()?;
    if let Some(build) = find_build_by_parameter(&client, &jobname, params)? {
        let console = build.get_console(&client)?;
        print!("{}", console);
    }
    Ok(())
}

/// Print the consoleText from a specific deployment nr for a service in a give region
pub fn specific_console(jobname: &str, nr: u32, params: &JobParams) -> Result<()> {
    let client = get_client()?;
    if let Some(build) = find_build_by_nr(&client, &jobname, nr, params)? {
        let console = build.get_console(&client)?;
        print!("{}", console);
    }
    Ok(())
}
