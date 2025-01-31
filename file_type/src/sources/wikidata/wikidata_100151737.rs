use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100151737: FileFormat = FileFormat {
    id: 100_151_737,
    puid: "wikidata/100151737",
    name: "Muvee autoProducer Project File",
    extensions: &["mve"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
