use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205901: FileFormat = FileFormat {
    id: 28_205_901,
    source_type: SourceType::Wikidata,
    name: "DGI",
    extensions: &["dgi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
