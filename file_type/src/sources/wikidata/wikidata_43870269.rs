use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_43870269: FileFormat = FileFormat {
    id: 43_870_269,
    source_type: SourceType::Wikidata,
    name: "PCX, version 4",
    extensions: &["pcc", "pcx"],
    media_types: &["image/vnd.zbrush.pcx", "image/x-pcx"],
    internal_signatures: &[],
    related_formats: &[],
};
