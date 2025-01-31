use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51837664: FileFormat = FileFormat {
    id: 51_837_664,
    puid: "wikidata/51837664",
    name: "Micrografx Designer format",
    extensions: &["dsf"],
    media_types: &["application/x-mgx-designer"],
    internal_signatures: &[],
    related_formats: &[],
};
