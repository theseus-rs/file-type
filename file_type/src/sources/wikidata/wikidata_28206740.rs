use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206740: FileFormat = FileFormat {
    id: 28_206_740,
    source_type: SourceType::Wikidata,
    name: "Navy Image File Format",
    extensions: &["ct3", "nif"],
    media_types: &["image/x-niff"],
    signatures: &[],
    related_formats: &[],
};
