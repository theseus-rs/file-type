use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121921117: FileFormat = FileFormat {
    id: 121_921_117,
    source_type: SourceType::Wikidata,
    name: "Ptex File Format",
    extensions: &["ptx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
