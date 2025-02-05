use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960128: FileFormat = FileFormat {
    id: 27_960_128,
    source_type: SourceType::Wikidata,
    name: "Computerized Speech Lab NSP",
    extensions: &["nsp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
