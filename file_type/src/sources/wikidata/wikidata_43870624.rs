use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_43870624: FileFormat = FileFormat {
    id: 43_870_624,
    source_type: SourceType::Wikidata,
    name: "PCX, version 5",
    extensions: &["pcc", "pcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
