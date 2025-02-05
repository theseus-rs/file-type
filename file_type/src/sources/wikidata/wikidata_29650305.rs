use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650305: FileFormat = FileFormat {
    id: 29_650_305,
    source_type: SourceType::Wikidata,
    name: "PSI-MI XML",
    extensions: &["dag", "def"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
