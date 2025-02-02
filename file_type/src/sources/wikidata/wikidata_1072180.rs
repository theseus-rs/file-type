use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1072180: FileFormat = FileFormat {
    id: 1_072_180,
    source_type: SourceType::Wikidata,
    name: "Synchronized Multimedia Integration Language",
    extensions: &["smi", "smil"],
    media_types: &["application/smil+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
