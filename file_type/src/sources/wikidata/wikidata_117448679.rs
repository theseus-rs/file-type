use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117448679: FileFormat = FileFormat {
    id: 117_448_679,
    source_type: SourceType::Wikidata,
    name: "Praat TextGrid",
    extensions: &["textgrid"],
    media_types: &["text/praat-textgrid"],
    signatures: &[],
    related_formats: &[],
};
