use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2207671: FileFormat = FileFormat {
    id: 2_207_671,
    source_type: SourceType::Wikidata,
    name: "SQX",
    extensions: &["sqx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
