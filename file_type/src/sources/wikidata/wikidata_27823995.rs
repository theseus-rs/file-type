use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27823995: FileFormat = FileFormat {
    id: 27_823_995,
    puid: "wikidata/27823995",
    name: "Maptech KAPP image file, version 3.0",
    extensions: &["kap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
