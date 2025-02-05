use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857590: FileFormat = FileFormat {
    id: 105_857_590,
    source_type: SourceType::Wikidata,
    name: "ipuz puzzle open format",
    extensions: &["ipuz"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
