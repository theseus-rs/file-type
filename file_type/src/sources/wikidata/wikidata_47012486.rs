use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47012486: FileFormat = FileFormat {
    id: 47_012_486,
    source_type: SourceType::Wikidata,
    name: "MultiMate Text File",
    extensions: &["dox", "fnx", "pat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
