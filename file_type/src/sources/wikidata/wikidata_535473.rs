use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_535473: FileFormat = FileFormat {
    id: 535_473,
    source_type: SourceType::Wikidata,
    name: "PCX file format family",
    extensions: &["pcc", "pcx"],
    media_types: &["image/vnd.zbrush.pcx"],
    internal_signatures: &[],
    related_formats: &[],
};
