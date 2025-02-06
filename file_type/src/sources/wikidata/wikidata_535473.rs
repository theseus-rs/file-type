use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_535473: FileFormat = FileFormat {
    id: 535_473,
    source_type: SourceType::Wikidata,
    name: "PCX file format family",
    extensions: &["pcc", "pcx"],
    media_types: &["image/vnd.zbrush.pcx"],
    signatures: &[],
    related_formats: &[],
};
