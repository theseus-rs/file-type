use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130280165: FileFormat = FileFormat {
    id: 130_280_165,
    source_type: SourceType::Wikidata,
    name: "Mask file format",
    extensions: &["mask"],
    media_types: &["text/x-mask"],
    signatures: &[],
    related_formats: &[],
};
