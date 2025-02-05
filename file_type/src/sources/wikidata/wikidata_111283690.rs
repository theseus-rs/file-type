use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111283690: FileFormat = FileFormat {
    id: 111_283_690,
    source_type: SourceType::Wikidata,
    name: "Casio FZ-1 voice dump format",
    extensions: &["fzv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
