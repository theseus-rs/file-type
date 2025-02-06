use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_43870269: FileFormat = FileFormat {
    id: 43_870_269,
    source_type: SourceType::Wikidata,
    name: "PCX, version 4",
    extensions: &["pcc", "pcx"],
    media_types: &["image/vnd.zbrush.pcx", "image/x-pcx"],
    signatures: &[],
    related_formats: &[],
};
