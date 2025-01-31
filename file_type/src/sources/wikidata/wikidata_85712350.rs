use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85712350: FileFormat = FileFormat {
    id: 85_712_350,
    puid: "wikidata/85712350",
    name: "Calendar Creator File 7-8",
    extensions: &["bcc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
