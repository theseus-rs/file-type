use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117338260: FileFormat = FileFormat {
    id: 117_338_260,
    source_type: SourceType::Wikidata,
    name: "Corel Library",
    extensions: &["clb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
