pub(crate) fn round_trip<T>(xml: &str) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let value: T = quick_xml::de::from_str(xml)?;
    let xml = quick_xml::se::to_string(&value)?;
    Ok(quick_xml::de::from_str(&xml)?)
}
