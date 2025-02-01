use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_535473: FileFormat = FileFormat {
    id: 535_473,
    puid: "wikidata/535473",
    name: "PCX file format family",
    extensions: &["pcc", "pcx"],
    media_types: &["image/vnd.zbrush.pcx", "image/vnd.zbrush.pcx"],
    internal_signatures: &[],
    related_formats: &[],
};
