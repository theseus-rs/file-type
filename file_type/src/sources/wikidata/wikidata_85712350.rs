use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85712350: FileFormat = FileFormat {
    id: 85_712_350,
    source_type: SourceType::Wikidata,
    name: "Calendar Creator File 7-8",
    extensions: &["bcc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
