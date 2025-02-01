use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117448679: FileFormat = FileFormat {
    id: 117_448_679,
    puid: "wikidata/117448679",
    name: "Praat TextGrid",
    extensions: &["textgrid"],
    media_types: &["text/praat-textgrid"],
    internal_signatures: &[],
    related_formats: &[],
};
