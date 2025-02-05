use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205746: FileFormat = FileFormat {
    id: 28_205_746,
    source_type: SourceType::Wikidata,
    name: "Microsoft Fax At Work Document",
    extensions: &["awd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
