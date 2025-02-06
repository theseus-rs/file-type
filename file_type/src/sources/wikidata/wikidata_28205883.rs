use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205883: FileFormat = FileFormat {
    id: 28_205_883,
    source_type: SourceType::Wikidata,
    name: "Desktop Color Separation",
    extensions: &["c", "dcs", "eps", "k", "m", "y"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
