use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205835: FileFormat = FileFormat {
    id: 28_205_835,
    source_type: SourceType::Wikidata,
    name: "Cloé picture",
    extensions: &["clo", "cloe"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
