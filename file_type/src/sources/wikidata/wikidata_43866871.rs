use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_43866871: FileFormat = FileFormat {
    id: 43_866_871,
    source_type: SourceType::Wikidata,
    name: "PCX, version 2",
    extensions: &["pcc", "pcx"],
    media_types: &["image/vnd.zbrush.pcx", "image/x-pcx"],
    internal_signatures: &[],
    related_formats: &[],
};
