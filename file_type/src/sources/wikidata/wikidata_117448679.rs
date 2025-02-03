use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117448679: FileFormat = FileFormat {
    id: 117_448_679,
    source_type: SourceType::Wikidata,
    name: "Praat TextGrid",
    extensions: &["textgrid"],
    media_types: &["text/praat-textgrid"],
    internal_signatures: &[],
    related_formats: &[],
};
