use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850033: FileFormat = FileFormat {
    id: 105_850_033,
    source_type: SourceType::Wikidata,
    name: "Xilinx Core Generator System Project (with rem)",
    extensions: &["cgp"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
