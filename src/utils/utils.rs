use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_millis_now() -> u64 {
    let current_time: SystemTime = SystemTime::now();
    let duration_since_epoch: Duration = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Convert the duration to milliseconds
    let miliseconds: u64 =
        duration_since_epoch.as_secs() * 1000 + u64::from(duration_since_epoch.subsec_millis());
    return miliseconds;
}

pub fn create_kml(readings: String) -> String {
    return format!(
        "<?xml version='1.0' encoding='UTF-8'?>
<kml xmlns='http://www.opengis.net/kml/2.2'>
  <Document>
    <name>Flight Path</name>
    <Placemark>
      <name>Flight Track</name>
      <LineString>
        <extrude>1</extrude>
        <altitudeMode>absolute</altitudeMode>
        <coordinates>
          {}
        </coordinates>
      </LineString>
    </Placemark>
  </Document>
</kml>
",
        readings
    );
}
