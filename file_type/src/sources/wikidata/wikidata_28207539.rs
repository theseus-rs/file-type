use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207539: FileFormat = FileFormat {
    id: 28_207_539,
    source_type: SourceType::Wikidata,
    name: "Xerox EDMICS-MMR",
    extensions: &["ed", "mmr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
