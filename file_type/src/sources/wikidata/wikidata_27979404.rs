use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979404: FileFormat = FileFormat {
    id: 27_979_404,
    source_type: SourceType::Wikidata,
    name: "PICS",
    extensions: &["pcs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
