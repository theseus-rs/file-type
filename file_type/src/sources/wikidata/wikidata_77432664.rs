use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_77432664: FileFormat = FileFormat {
    id: 77_432_664,
    source_type: SourceType::Wikidata,
    name: "InfoPath Template Part",
    extensions: &["xtp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
