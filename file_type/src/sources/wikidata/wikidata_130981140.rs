use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130981140: FileFormat = FileFormat {
    id: 130_981_140,
    source_type: SourceType::Wikidata,
    name: "Slurm script file format",
    extensions: &["sl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
