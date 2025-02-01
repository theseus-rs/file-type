use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76158562: FileFormat = FileFormat {
    id: 76_158_562,
    puid: "wikidata/76158562",
    name: "VisKit 3d model",
    extensions: &["vk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
