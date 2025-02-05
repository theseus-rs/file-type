use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_81526528: FileFormat = FileFormat {
    id: 81_526_528,
    source_type: SourceType::Wikidata,
    name: "MicroStation Resource data",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
