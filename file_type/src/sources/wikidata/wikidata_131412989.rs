use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131412989: FileFormat = FileFormat {
    id: 131_412_989,
    puid: "wikidata/131412989",
    name: "VisualProlog file format",
    extensions: &["cl", "i", "pack", "ph", "pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
