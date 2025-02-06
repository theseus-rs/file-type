use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116784988: FileFormat = FileFormat {
    id: 116_784_988,
    source_type: SourceType::Wikidata,
    name: "Project Manager Pro Template file",
    extensions: &["pmt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
