use crate::format::FileFormat;

pub(crate) const LINGUIST_98384424: FileFormat = FileFormat {
    id: 98_384_424,
    puid: "linguist/98384424",
    name: "iCalendar",
    extensions: &["ical", "ics"],
    media_types: &["text/x-properties"],
    internal_signatures: &[],
    related_formats: &[],
};
