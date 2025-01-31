use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125208050: FileFormat = FileFormat {
    id: 125_208_050,
    puid: "wikidata/125208050",
    name: "Microsoft Project XML",
    extensions: &["mspxml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
