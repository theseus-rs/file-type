use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111283690: FileFormat = FileFormat {
    id: 111_283_690,
    puid: "wikidata/111283690",
    name: "Casio FZ-1 voice dump format",
    extensions: &["fzv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
