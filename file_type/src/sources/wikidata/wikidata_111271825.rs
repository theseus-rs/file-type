use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111271825: FileFormat = FileFormat {
    id: 111_271_825,
    source_type: SourceType::Wikidata,
    name: "Yamaha DX7 voice SysEx dump",
    extensions: &["dx7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
