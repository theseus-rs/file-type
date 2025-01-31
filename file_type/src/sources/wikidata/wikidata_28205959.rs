use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205959: FileFormat = FileFormat {
    id: 28_205_959,
    puid: "wikidata/28205959",
    name: "Digital Video Interactive Red Channel",
    extensions: &["imr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
