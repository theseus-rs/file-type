use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130981140: FileFormat = FileFormat {
    id: 130_981_140,
    puid: "wikidata/130981140",
    name: "Slurm script file format",
    extensions: &["sl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
