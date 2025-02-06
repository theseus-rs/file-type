use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117851210: FileFormat = FileFormat {
    id: 117_851_210,
    source_type: SourceType::Wikidata,
    name: "Tektronix Plot 10 file",
    extensions: &["p10"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
