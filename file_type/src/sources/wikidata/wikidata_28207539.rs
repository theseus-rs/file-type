use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207539: FileFormat = FileFormat {
    id: 28_207_539,
    source_type: SourceType::Wikidata,
    name: "Xerox EDMICS-MMR",
    extensions: &["ed", "mmr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
