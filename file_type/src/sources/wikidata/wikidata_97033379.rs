use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_97033379: FileFormat = FileFormat {
    id: 97_033_379,
    puid: "wikidata/97033379",
    name: "Magick Persistent Registry",
    extensions: &["mpr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
