use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959848: FileFormat = FileFormat {
    id: 27_959_848,
    source_type: SourceType::Wikidata,
    name: "LMMS music file",
    extensions: &["mmp", "mmpz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
