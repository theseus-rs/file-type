use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851939: FileFormat = FileFormat {
    id: 105_851_939,
    puid: "wikidata/105851939",
    name: "GIMP Script-Fu Script",
    extensions: &["scm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
