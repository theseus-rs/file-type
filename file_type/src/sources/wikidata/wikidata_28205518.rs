use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205518: FileFormat = FileFormat {
    id: 28_205_518,
    puid: "wikidata/28205518",
    name: "atomix.scores",
    extensions: &["scores"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
