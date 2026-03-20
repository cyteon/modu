## Time
The modu time library comes with several functions to work with time, including a `sleep(ms)` function,
but this wont work here as its unsupported on web

[CODE]
import "std/time";

// prints the current time in unix seconds
print(time.now_unix());

// prints the current time in unix milliseconds
print(time.now_unix_ms());

// prints the current utc time
print(time.now_utc());

// prints the current local time
print(time.now_local());

// prints the iso 8601 timestamp for the current unix time
print(time.to_iso_8601(time.now_unix()));

// prints the rfc 2822 timstamp for the current unix time
print(time.to_rfc_2822(time.now_unix()));

// prints the local datetime for the current unix time
print(time.to_local_date_time(time.now_unix()));

// prints the utc datetime for the current unix time
print(time.to_utc_date_time(time.now_unix()));

// sleep(ms) is unsupported on web
// time.sleep(2000);
// print("i waited 2s");