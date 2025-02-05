use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66811836: FileFormat = FileFormat {
    id: 66_811_836,
    source_type: SourceType::Wikidata,
    name: "Inform source code file",
    extensions: &["inf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
