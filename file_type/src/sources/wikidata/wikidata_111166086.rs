use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111166086: FileFormat = FileFormat {
    id: 111_166_086,
    puid: "wikidata/111166086",
    name: "Songbase File",
    extensions: &["sngbase"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
