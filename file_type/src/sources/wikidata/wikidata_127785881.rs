use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127785881: FileFormat = FileFormat {
    id: 127_785_881,
    puid: "wikidata/127785881",
    name: "Modula-2 file",
    extensions: &["mod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
